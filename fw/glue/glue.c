#include "glue.h"
#include "coresynth.h"

#include <string.h>

void system_reset() {
    NVIC_SystemReset();
}

void HAL_GPIO_EXTI_Callback(uint16_t pin) {

}

void set_pwm(float value, TIM_HandleTypeDef* tim, uint32_t channel) {
    TIM_OC_InitTypeDef sConfigOC;
  
    sConfigOC.OCMode = TIM_OCMODE_PWM1;
    sConfigOC.Pulse = (uint16_t)(tim->Init.Period * value);
    sConfigOC.OCPolarity = TIM_OCPOLARITY_HIGH;
    sConfigOC.OCFastMode = TIM_OCFAST_DISABLE;
    HAL_TIM_PWM_ConfigChannel(tim, &sConfigOC, channel);
    HAL_TIM_PWM_Start(tim, channel);
}

extern UART_HandleTypeDef huart1;

void print_value(char* str) {
    size_t len = strlen(str);

    HAL_UART_Transmit(&huart1, (uint8_t*)str, (uint16_t)len, HAL_MAX_DELAY);
}
