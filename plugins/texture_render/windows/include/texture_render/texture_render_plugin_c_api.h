#ifndef FLUTTER_PLUGIN_TEXTURE_RENDER_PLUGIN_C_API_H_
#define FLUTTER_PLUGIN_TEXTURE_RENDER_PLUGIN_C_API_H_

#include <flutter_plugin_registrar.h>

#ifdef FLUTTER_PLUGIN_IMPL
#define FLUTTER_PLUGIN_EXPORT __declspec(dllexport)
#else
#define FLUTTER_PLUGIN_EXPORT __declspec(dllimport)
#endif

#if defined(__cplusplus)
extern "C" {
#endif

FLUTTER_PLUGIN_EXPORT void TextureRenderPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar);

FLUTTER_PLUGIN_EXPORT void UpdateFrameCallback(int64_t texture_id,
                                               void *video_texture_ptr,
                                               void *new_frame_ptr);

#if defined(__cplusplus)
} // extern "C"
#endif

#endif // FLUTTER_PLUGIN_TEXTURE_RENDER_PLUGIN_C_API_H_
