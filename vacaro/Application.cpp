#include "Application.h"
#include "imgui.h"

namespace Vacaro {
    void RenderUI() {
        ImGui::Begin("Settings");
        ImGui::Button("Hello");
        static float value = 0.0;
        ImGui::DragFloat("Value", &value);
        ImGui::End();

        ImGui::ShowDemoWindow();
    }
}
