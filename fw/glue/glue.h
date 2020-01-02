#pragma once

#include <stdint.h>
#include "stm32f1xx_hal.h"
#include "cmsis_os.h"

void set_pwm(float value, TIM_HandleTypeDef* tim, uint32_t channel);

void system_reset();

void gpio_init(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin, uint32_t mode);

// TODO for some reason, bindgen wont generate osKernelSysTickFrequency
const uint32_t TICKS_FREQ = osKernelSysTickFrequency;
