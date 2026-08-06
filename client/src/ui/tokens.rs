pub const FOCUS_RING: &str = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-amber-400 focus-visible:ring-offset-1 rounded-xl";

pub const LBL: &str = "block text-[11px] font-semibold tracking-wider uppercase text-amber-800/80 mb-1.5";

pub const NATIVE_INPUT: &str = "w-full h-10 px-3.5 py-2 rounded-xl border border-amber-200/90 bg-amber-50/30 text-sm text-zinc-800 placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-amber-400/50 focus:border-amber-400 focus:bg-white transition-all duration-150";

// Wird in overview.rs benötigt
pub const SAVE_GLOW_CSS: &str = r#"
@keyframes save-glow {
    0%   { border-color: #d1fae5; box-shadow: 0 0 0 0px rgba(34, 197, 94, 0), 0 1px 3px 0 rgba(0, 0, 0, 0.06); }
    20%  { border-color: #22c55e; box-shadow: 0 0 0 5px rgba(34, 197, 94, 0.22), 0 1px 3px 0 rgba(0, 0, 0, 0.06); }
    55%  { border-color: #16a34a; box-shadow: 0 0 0 5px rgba(34, 197, 94, 0.10), 0 1px 3px 0 rgba(0, 0, 0, 0.06); }
    100% { border-color: #bbf7d0; box-shadow: 0 0 0 0px rgba(34, 197, 94, 0), 0 1px 3px 0 rgba(0, 0, 0, 0.06); }
}
.save-glow-card { animation: save-glow 2s ease-in-out forwards; }
.save-glow-card .save-glow-header { background-color: rgba(240, 253, 244, 0.70) !important; border-bottom-color: #bbf7d0 !important; transition: background-color 0.4s ease, border-color 0.4s ease; }
.save-glow-card .save-glow-accent { background-color: rgba(34, 197, 94, 0.75) !important; transition: background-color 0.4s ease; }
.save-glow-card .save-glow-title { color: #166534 !important; transition: color 0.4s ease; }
"#;