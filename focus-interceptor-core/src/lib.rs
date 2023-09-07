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

mod movement_monitor {
    use std::{
        collections::VecDeque,
        time::{Duration, Instant},
    };

    #[derive(Debug)]
    struct OccurrenceLogger {
        timestamps: VecDeque<Instant>,
        pub max_data: usize,
    }

    impl OccurrenceLogger {
        pub fn new() -> OccurrenceLogger {
            OccurrenceLogger::default()
        }

        pub fn new_with_max_data(max_data: usize) -> OccurrenceLogger {
            OccurrenceLogger {
                max_data,
                ..Default::default()
            }
        }

        fn log_now(&mut self) {
            self.timestamps.push_front(Instant::now());
            self.timestamps.truncate(self.max_data);
        }

        fn count_in_time_window(&self, time_window: Duration) -> usize {
            let latest_time = self.timestamps[0];
            let earliest_time = latest_time - time_window;
            self.timestamps
                .iter()
                .filter(|&&timestamp| timestamp >= earliest_time)
                .count()
        }

        fn rate_in_time_window(&self, time_window: Duration) -> f64 {
            self.count_in_time_window(time_window) as f64 / time_window.as_secs_f64()
        }
    }

    impl Default for OccurrenceLogger {
        fn default() -> Self {
            OccurrenceLogger {
                timestamps: VecDeque::default(),
                max_data: usize::MAX,
            }
        }
    }
}
