use near_sdk_sim::ExecutionResult;
use near_sdk_sim::transaction::ExecutionStatus;

pub fn assert_one_promise_error(promise_result: ExecutionResult, expected_error_message: &str) {
  assert_eq!(promise_result.promise_errors().len(), 1);

  if let ExecutionStatus::Failure(execution_error) =
      &promise_result.promise_errors().remove(0).unwrap().outcome().status
  {
    assert!(execution_error.to_string().contains(expected_error_message));
  } else {
    unreachable!();
  }
}
