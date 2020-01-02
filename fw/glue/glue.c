#include "glue.h"
#include "coresynth.h"

#include <string.h>

const uint32_t TICKS_FREQ = osKernelSysTickFrequency;

void system_reset() {
    NVIC_SystemReset();
}

void HAL_GPIO_EXTI_Callback(uint16_t pin) {
    if(pin == GPIO_PIN_10 || pin == GPIO_PIN_10) {
        handle_input(
            HAL_GPIO_ReadPin(GPIOB, GPIO_PIN_10) == GPIO_PIN_SET ? 1 : 0,
            HAL_GPIO_ReadPin(GPIOB, GPIO_PIN_11) == GPIO_PIN_SET ? 1 : 0
        );
    }
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

void gpio_init(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin, uint32_t mode) {
    GPIO_InitTypeDef GPIO_InitStruct;

    GPIO_InitStruct.Pin = GPIO_Pin;
    GPIO_InitStruct.Pull = GPIO_NOPULL;
    GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_HIGH;
    GPIO_InitStruct.Mode = mode;

    HAL_GPIO_Init(GPIOx, &GPIO_InitStruct);
}
