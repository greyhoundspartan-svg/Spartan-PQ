/* Minimal stub for MSVC builds - full Greyhound/Labrador requires GCC/clang.
 * This produces libdogs.lib so linking succeeds. Stub PCS is used at runtime.
 */
#ifdef _MSC_VER
/* Prevent "empty translation unit" warning */
static int msvc_stub_dogs_dummy = 0;
#endif
