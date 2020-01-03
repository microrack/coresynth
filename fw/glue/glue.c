#include "glue.h"
#include "coresynth.h"

#include <string.h>

void system_reset() {
    NVIC_SystemReset();
}

uint32_t BUTTON_GPIOS[] = {
    GPIO_PIN_0,
    GPIO_PIN_1,
    GPIO_PIN_2,
    GPIO_PIN_3,
    GPIO_PIN_4,
    GPIO_PIN_5,
    GPIO_PIN_6,
    GPIO_PIN_7,
    GPIO_PIN_8,
    GPIO_PIN_12
};

void HAL_GPIO_EXTI_Callback(uint16_t pin) {
    if(pin == GPIO_PIN_10 || pin == GPIO_PIN_11) {
        handle_input(
            HAL_GPIO_ReadPin(GPIOB, GPIO_PIN_10) == GPIO_PIN_SET ? 1 : 0,
            HAL_GPIO_ReadPin(GPIOB, GPIO_PIN_11) == GPIO_PIN_SET ? 1 : 0
        );
    }

    size_t id = 0;

    for(id = 0; id < 10; id++) {
        if(pin == BUTTON_GPIOS[id]) {
            handle_button(HAL_GPIO_ReadPin(GPIOA, BUTTON_GPIOS[id]) == GPIO_PIN_SET ? 1 : 0, id);
            break;
        }
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
