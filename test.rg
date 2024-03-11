//! Test file

//! Type Declarations
//! <Optional_Attribute> <Label> <Type>

//! Local Declarations
//! <Optional_Attribute> <Label> <Optional_Type> = <Value>

pub main fn() {
    mut a = 5
    mut c Color = Color{r: 127, g: 127, b: 127}
}

MyColor enum {
    Red,
    Green,
    Blue,
}

Color struct {
    pub r u8,
    pub g u8,
    pub b u8,
}

add fn(mut a $int, b int) int {
            
}

AliasColor alias Color {r: 0, g: 0, b: 0}

COLOR_RED = MyColor:Red

#run {
    assert(1 + 2 == 3)
}
