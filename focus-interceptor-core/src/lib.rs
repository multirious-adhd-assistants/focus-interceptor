#![allow(unused)]

mod schedule {
    use std::{collections::BTreeMap, time::Duration};

    #[derive(Debug, thiserror::Error)]
    pub enum StartScheduleError {
        #[error("Schedule not found")]
        NotFound,
        #[error("Schedule is already running")]
        AlreadyRunning,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum StopScheduleError {
        #[error("Schedule is not running")]
        NotRunning,
    }

    #[derive(Debug, Default, Clone)]
    pub struct Scheduler {
        schedules: BTreeMap<String, Schedule>,
        running_schedules: BTreeMap<String, ScheduleState>,
    }

    impl Scheduler {
        pub fn new() -> Scheduler {
            Scheduler::default()
        }

        pub fn insert_schedule(&mut self, title: String, schedule: Schedule) -> Option<Schedule> {
            self.schedules.insert(title, schedule)
        }

        pub fn start_schedule(&mut self, title: &str) -> Result<(), StartScheduleError> {
            if self.running_schedules.contains_key(title) {
                return Err(StartScheduleError::AlreadyRunning);
            }
            match self.schedules.get(title) {
                Some(s) => {
                    self.running_schedules
                        .insert(title.to_string(), ScheduleState::from_schedule(s));
                }
                None => return Err(StartScheduleError::NotFound),
            }
            Ok(())
        }

        pub fn stop_schedule(&mut self, title: &str) -> Result<(), StopScheduleError> {
            match self.running_schedules.remove(title) {
                Some(_s) => {
                    todo!();
                    Ok(())
                }
                None => Err(StopScheduleError::NotRunning),
            }
        }
    }

    /// State of the current running schedule
    #[derive(Debug, Default, Clone)]
    pub struct ScheduleState {
        tasks: Vec<Task>,
        on_stop_tasks: Vec<Task>,
        current_task: usize,
    }

    impl ScheduleState {
        fn from_schedule(schedule: &Schedule) -> ScheduleState {
            ScheduleState {
                tasks: schedule.tasks.clone(),
                on_stop_tasks: schedule.on_stop_tasks.clone(),
                current_task: 0,
            }
        }
    }

    /// Definition for a schedule
    #[derive(Debug, Default, Clone)]
    pub struct Schedule {
        /// The schedule will be initiated by the events specified.
        /// If no event is specified then the schedule can only be started manually.
        trigger: (),
        tasks: Vec<Task>,
        on_stop_tasks: Vec<Task>,
    }

    #[derive(Debug, Clone)]
    enum Task {
        Wait(Duration),
        Break(bool),
    }
}

mod action_monitor {
    pub const AVERAGE_CLICKS_PER_MINUTE: usize = 300;

    use std::{
        collections::{BTreeMap, VecDeque},
        time::{Duration, Instant},
    };

    #[derive(Debug, Default)]
    pub struct OccurrenceLogger {
        timestamps: VecDeque<Instant>,
    }

    impl OccurrenceLogger {
        fn new() -> OccurrenceLogger {
            OccurrenceLogger::default()
        }

        /// Shortens the deque, keeping the first `len` elements and dropping
        /// the rest.
        ///
        /// If `len` is greater than the deque's current length, this has no
        /// effect.
        fn truncate(&mut self, len: usize) {
            self.timestamps.truncate(len)
        }

        fn log_now(&mut self) {
            self.timestamps.push_front(Instant::now());
        }

        pub fn count_in_time_window(&self, time_window: Duration) -> usize {
            let latest_time = self.timestamps[0];
            let earliest_time = latest_time - time_window;
            self.timestamps
                .iter()
                .filter(|&&timestamp| timestamp >= earliest_time)
                .count()
        }

        pub fn rate_in_time_window(&self, time_window: Duration) -> f64 {
            self.count_in_time_window(time_window) as f64 / time_window.as_secs_f64()
        }
    }

    pub struct ActionMonitor {
        /// first u8 meant body part
        /// second u8 meant movement kind
        actions: BTreeMap<u8, BTreeMap<u8, OccurrenceLogger>>,
    }

    impl ActionMonitor {
        pub fn log_action(&mut self, body_part: u8, movement_kind: u8) {
            self.actions
                .entry(body_part)
                .or_insert_with(BTreeMap::new)
                .entry(movement_kind)
                .or_insert_with(OccurrenceLogger::new)
                .log_now()
        }

        pub fn purge_data(&mut self, len: usize) {
            for action_logger in self.action_logggers_flatten_mut() {
                action_logger.logger.truncate(len);
            }
        }

        pub fn actions(&self) -> &BTreeMap<u8, BTreeMap<u8, OccurrenceLogger>> {
            &self.actions
        }

        pub fn action_loggers_flatten(&self) -> impl Iterator<Item = ActionLoggerRef<'_>> {
            self.actions.iter().flat_map(|(body_part, m)| {
                m.iter().map(|(movement_kind, logger)| ActionLoggerRef {
                    body_part: *body_part,
                    movement_kind: *movement_kind,
                    logger,
                })
            })
        }

        fn action_logggers_flatten_mut(&mut self) -> impl Iterator<Item = ActionLoggerRefMut<'_>> {
            self.actions.iter_mut().flat_map(|(body_part, m)| {
                m.iter_mut()
                    .map(|(movement_kind, logger)| ActionLoggerRefMut {
                        body_part: *body_part,
                        movement_kind: *movement_kind,
                        logger,
                    })
            })
        }
    }

    pub struct ActionLoggerRef<'a> {
        pub body_part: u8,
        pub movement_kind: u8,
        pub logger: &'a OccurrenceLogger,
    }

    struct ActionLoggerRefMut<'a> {
        pub body_part: u8,
        pub movement_kind: u8,
        pub logger: &'a mut OccurrenceLogger,
    }
}
