#[doc = "Register `USBPHY_DM_CTRL` reader"]
pub type R = crate::R<USBPHY_DM_CTRL_SPEC>;
#[doc = "Register `USBPHY_DM_CTRL` writer"]
pub type W = crate::W<USBPHY_DM_CTRL_SPEC>;
#[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL  

Value on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FUNCSEL_A {
    #[doc = "2: `10`"]
    UART1_RX = 2,
    #[doc = "3: `11`"]
    I2C0_SCL = 3,
    #[doc = "5: `101`"]
    SIOB_PROC_57 = 5,
    #[doc = "31: `11111`"]
    NULL = 31,
}
impl From<FUNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNCSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FUNCSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for FUNCSEL_A {}
#[doc = "Field `FUNCSEL` reader - 0-31 -> selects pin function according to the gpio table 31 == NULL"]
pub type FUNCSEL_R = crate::FieldReader<FUNCSEL_A>;
impl FUNCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FUNCSEL_A> {
        match self.bits {
            2 => Some(FUNCSEL_A::UART1_RX),
            3 => Some(FUNCSEL_A::I2C0_SCL),
            5 => Some(FUNCSEL_A::SIOB_PROC_57),
            31 => Some(FUNCSEL_A::NULL),
            _ => None,
        }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == FUNCSEL_A::UART1_RX
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_i2c0_scl(&self) -> bool {
        *self == FUNCSEL_A::I2C0_SCL
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_siob_proc_57(&self) -> bool {
        *self == FUNCSEL_A::SIOB_PROC_57
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        *self == FUNCSEL_A::NULL
    }
}
#[doc = "Field `FUNCSEL` writer - 0-31 -> selects pin function according to the gpio table 31 == NULL"]
pub type FUNCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, FUNCSEL_A>;
impl<'a, REG> FUNCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCSEL_A::UART1_RX)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn i2c0_scl(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCSEL_A::I2C0_SCL)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn siob_proc_57(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCSEL_A::SIOB_PROC_57)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCSEL_A::NULL)
    }
}
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTOVER_A {
    #[doc = "0: drive output from peripheral signal selected by funcsel"]
    NORMAL = 0,
    #[doc = "1: drive output from inverse of peripheral signal selected by funcsel"]
    INVERT = 1,
    #[doc = "2: drive output low"]
    LOW = 2,
    #[doc = "3: drive output high"]
    HIGH = 3,
}
impl From<OUTOVER_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTOVER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OUTOVER_A {
    type Ux = u8;
}
impl crate::IsEnum for OUTOVER_A {}
#[doc = "Field `OUTOVER` reader - "]
pub type OUTOVER_R = crate::FieldReader<OUTOVER_A>;
impl OUTOVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUTOVER_A {
        match self.bits {
            0 => OUTOVER_A::NORMAL,
            1 => OUTOVER_A::INVERT,
            2 => OUTOVER_A::LOW,
            3 => OUTOVER_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "drive output from peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OUTOVER_A::NORMAL
    }
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == OUTOVER_A::INVERT
    }
    #[doc = "drive output low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUTOVER_A::LOW
    }
    #[doc = "drive output high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUTOVER_A::HIGH
    }
}
#[doc = "Field `OUTOVER` writer - "]
pub type OUTOVER_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OUTOVER_A, crate::Safe>;
impl<'a, REG> OUTOVER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "drive output from peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(OUTOVER_A::NORMAL)
    }
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(OUTOVER_A::INVERT)
    }
    #[doc = "drive output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(OUTOVER_A::LOW)
    }
    #[doc = "drive output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(OUTOVER_A::HIGH)
    }
}
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OEOVER_A {
    #[doc = "0: drive output enable from peripheral signal selected by funcsel"]
    NORMAL = 0,
    #[doc = "1: drive output enable from inverse of peripheral signal selected by funcsel"]
    INVERT = 1,
    #[doc = "2: disable output"]
    DISABLE = 2,
    #[doc = "3: enable output"]
    ENABLE = 3,
}
impl From<OEOVER_A> for u8 {
    #[inline(always)]
    fn from(variant: OEOVER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OEOVER_A {
    type Ux = u8;
}
impl crate::IsEnum for OEOVER_A {}
#[doc = "Field `OEOVER` reader - "]
pub type OEOVER_R = crate::FieldReader<OEOVER_A>;
impl OEOVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OEOVER_A {
        match self.bits {
            0 => OEOVER_A::NORMAL,
            1 => OEOVER_A::INVERT,
            2 => OEOVER_A::DISABLE,
            3 => OEOVER_A::ENABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OEOVER_A::NORMAL
    }
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == OEOVER_A::INVERT
    }
    #[doc = "disable output"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OEOVER_A::DISABLE
    }
    #[doc = "enable output"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OEOVER_A::ENABLE
    }
}
#[doc = "Field `OEOVER` writer - "]
pub type OEOVER_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OEOVER_A, crate::Safe>;
impl<'a, REG> OEOVER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(OEOVER_A::NORMAL)
    }
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(OEOVER_A::INVERT)
    }
    #[doc = "disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OEOVER_A::DISABLE)
    }
    #[doc = "enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OEOVER_A::ENABLE)
    }
}
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INOVER_A {
    #[doc = "0: don't invert the peri input"]
    NORMAL = 0,
    #[doc = "1: invert the peri input"]
    INVERT = 1,
    #[doc = "2: drive peri input low"]
    LOW = 2,
    #[doc = "3: drive peri input high"]
    HIGH = 3,
}
impl From<INOVER_A> for u8 {
    #[inline(always)]
    fn from(variant: INOVER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INOVER_A {
    type Ux = u8;
}
impl crate::IsEnum for INOVER_A {}
#[doc = "Field `INOVER` reader - "]
pub type INOVER_R = crate::FieldReader<INOVER_A>;
impl INOVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INOVER_A {
        match self.bits {
            0 => INOVER_A::NORMAL,
            1 => INOVER_A::INVERT,
            2 => INOVER_A::LOW,
            3 => INOVER_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "don't invert the peri input"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == INOVER_A::NORMAL
    }
    #[doc = "invert the peri input"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == INOVER_A::INVERT
    }
    #[doc = "drive peri input low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == INOVER_A::LOW
    }
    #[doc = "drive peri input high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == INOVER_A::HIGH
    }
}
#[doc = "Field `INOVER` writer - "]
pub type INOVER_W<'a, REG> = crate::FieldWriter<'a, REG, 2, INOVER_A, crate::Safe>;
impl<'a, REG> INOVER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "don't invert the peri input"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(INOVER_A::NORMAL)
    }
    #[doc = "invert the peri input"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(INOVER_A::INVERT)
    }
    #[doc = "drive peri input low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(INOVER_A::LOW)
    }
    #[doc = "drive peri input high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(INOVER_A::HIGH)
    }
}
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRQOVER_A {
    #[doc = "0: don't invert the interrupt"]
    NORMAL = 0,
    #[doc = "1: invert the interrupt"]
    INVERT = 1,
    #[doc = "2: drive interrupt low"]
    LOW = 2,
    #[doc = "3: drive interrupt high"]
    HIGH = 3,
}
impl From<IRQOVER_A> for u8 {
    #[inline(always)]
    fn from(variant: IRQOVER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IRQOVER_A {
    type Ux = u8;
}
impl crate::IsEnum for IRQOVER_A {}
#[doc = "Field `IRQOVER` reader - "]
pub type IRQOVER_R = crate::FieldReader<IRQOVER_A>;
impl IRQOVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IRQOVER_A {
        match self.bits {
            0 => IRQOVER_A::NORMAL,
            1 => IRQOVER_A::INVERT,
            2 => IRQOVER_A::LOW,
            3 => IRQOVER_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "don't invert the interrupt"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IRQOVER_A::NORMAL
    }
    #[doc = "invert the interrupt"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == IRQOVER_A::INVERT
    }
    #[doc = "drive interrupt low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IRQOVER_A::LOW
    }
    #[doc = "drive interrupt high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IRQOVER_A::HIGH
    }
}
#[doc = "Field `IRQOVER` writer - "]
pub type IRQOVER_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IRQOVER_A, crate::Safe>;
impl<'a, REG> IRQOVER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "don't invert the interrupt"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(IRQOVER_A::NORMAL)
    }
    #[doc = "invert the interrupt"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(IRQOVER_A::INVERT)
    }
    #[doc = "drive interrupt low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(IRQOVER_A::LOW)
    }
    #[doc = "drive interrupt high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(IRQOVER_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:4 - 0-31 -> selects pin function according to the gpio table 31 == NULL"]
    #[inline(always)]
    pub fn funcsel(&self) -> FUNCSEL_R {
        FUNCSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn outover(&self) -> OUTOVER_R {
        OUTOVER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn oeover(&self) -> OEOVER_R {
        OEOVER_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn inover(&self) -> INOVER_R {
        INOVER_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn irqover(&self) -> IRQOVER_R {
        IRQOVER_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0-31 -> selects pin function according to the gpio table 31 == NULL"]
    #[inline(always)]
    #[must_use]
    pub fn funcsel(&mut self) -> FUNCSEL_W<USBPHY_DM_CTRL_SPEC> {
        FUNCSEL_W::new(self, 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn outover(&mut self) -> OUTOVER_W<USBPHY_DM_CTRL_SPEC> {
        OUTOVER_W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn oeover(&mut self) -> OEOVER_W<USBPHY_DM_CTRL_SPEC> {
        OEOVER_W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn inover(&mut self) -> INOVER_W<USBPHY_DM_CTRL_SPEC> {
        INOVER_W::new(self, 16)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn irqover(&mut self) -> IRQOVER_W<USBPHY_DM_CTRL_SPEC> {
        IRQOVER_W::new(self, 28)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`usbphy_dm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphy_dm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPHY_DM_CTRL_SPEC;
impl crate::RegisterSpec for USBPHY_DM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphy_dm_ctrl::R`](R) reader structure"]
impl crate::Readable for USBPHY_DM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbphy_dm_ctrl::W`](W) writer structure"]
impl crate::Writable for USBPHY_DM_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBPHY_DM_CTRL to value 0x1f"]
impl crate::Resettable for USBPHY_DM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1f;
}
