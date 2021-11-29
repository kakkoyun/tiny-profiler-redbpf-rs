#![no_std]
#![no_main]

use cty::*;
use redbpf_probes::helpers::*;

// use one of the preludes
// use redbpf_probes::kprobe::prelude::*;
// use redbpf_probes::xdp::prelude::*;
// use redbpf_probes::socket_filter::prelude::*;

// Use the types you're going to share with userspace, eg:
// use probes::agent::SomeEvent;

program!(0xFFFFFFFE, "GPL");

// The maps and probe functions go here, eg:
//
// #[map("syscall_events")]
// static mut syscall_events: PerfMap<SomeEvent> = PerfMap::with_max_entries(1024);
//
// #[kprobe("__x64_sys_open")]
// fn syscall_enter(regs: Registers) {
//   let pid_tgid = bpf_get_current_pid_tgid();
//   ...
//
//   let event = SomeEvent {
//     pid: pid_tgid >> 32,
//     ...
//   };
//   unsafe { syscall_events.insert(regs.ctx, &event) };
// }

const MAX_STACK_DEPTH: usize = 127;

#[map]
static STACK_TRACES: HashMap<[u64; MAX_STACK_DEPTH], u32> = HashMap::with_max_entries(1024);

#[map]
static mut counts: PerfMap<u64> = PerfMap::with_max_entries(10240);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct StackCount {
    pub pid: u32,
    pub user_stack_id: usize,
    pub kernel_stack_id: usize,
}


#[perf_event]
fn do_sample(args: &bpf_perf_event_data) {
    let id = bpf_get_current_pid_tgid();
    let tgid = id >> 32;
    let pid = id;

    if pid == 0 { return; }

    let key = StackCount{pid: tgid, user_stack_id: 0, kernel_stack_id: 0 };

    counts.

    // let mut stack = [0; MAX_STACK_DEPTH];
    // // backtrace(regs, &mut stack);
    // let mut count = STACK_TRACES.get(&stack).unwrap_or_default();
    // count += 1;
    // STACK_TRACES.insert(&stack, &count);
}


//  // create map key
//  stack_count_key_t key = { .pid = tgid };
//
//  // get stacks
//  key.user_stack_id = bpf_get_stackid (ctx, &stack_traces, BPF_F_USER_STACK);
//  key.kernel_stack_id = bpf_get_stackid (ctx, &stack_traces, 0);
//
//  u64 zero = 0;
//  u64 *count;
//  count = bpf_map_lookup_or_try_init (&counts, &key, &zero);
//  if (!count)
//    return 0;
//
//  __sync_fetch_and_add (count, 1);
//
//  return 0;
