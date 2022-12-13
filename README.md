<div align="center">


# GDP
Genshin Data Parser


</div>

## Introduction
*GDP* is a query language for **genshindata** parsing, where itself does not contain any data.  
With GDP, you can query data like this:
```
// select a weapon's data where its CHS name equals to "黑剑"
WeaponExcelConfigData.nameTextMapHash ?weapon ?hash && CHS ?hash "黑剑"
```
Output:
```
[
  {
    "?hash": "3796905611",
    "?weapon": {
      "awakenCosts": [
        1000,
        2000,
        4000,
        8000
      ],
      "awakenIcon": "UI_EquipIcon_Sword_Bloodstained_Awaken",
      "awakenLightMapTexture": "ART/Equip/AvatarEquip/Equip_Sword_Bloodstained/Equip_Sword_Bloodstained_OverrideTexture/Equip_Sword_Bloodstained_02_Tex_Lightmap",
      "awakenTexture": "ART/Equip/AvatarEquip/Equip_Sword_Bloodstained/Equip_Sword_Bloodstained_OverrideTexture/Equip_Sword_Bloodstained_02_Tex_Diffuse",
      "descTextMapHash": 1253880813,
      "destroyReturnMaterial": [
        0
      ],
      "destroyReturnMaterialCount": [
        0
      ],
      "gachaCardNameHashPre": 163,
      "gachaCardNameHashSuffix": 4058331488,
      "gadgetId": 50011409,
      "icon": "UI_EquipIcon_Sword_Bloodstained",
      "id": 11409,
      "initialLockState": 2,
      "itemType": "ITEM_WEAPON",
      "nameTextMapHash": 3796905611,
      "rank": 10,
      "rankLevel": 4,
      "skillAffix": [
        111409,
        0
      ],
      "storyId": 191409,
      "weaponBaseExp": 50000,
      "weaponPromoteId": 11409,
      "weaponProp": [
        {
          "initValue": 42.4010009765625,
          "propType": "FIGHT_PROP_BASE_ATTACK",
          "type": "GROW_CURVE_ATTACK_201"
        },
        {
          "initValue": 0.05999999865889549,
          "propType": "FIGHT_PROP_CRITICAL",
          "type": "GROW_CURVE_CRITICAL_201"
        }
      ],
      "weaponType": "WEAPON_SWORD_ONE_HAND",
      "weight": 1
    }
  }
]
```

## Usage
It can be used as a querying console
```bash
gdp --path <path-to-genshin-data>
```

Or can be used as a rust library

## Grammar
### File Query
A file query contains a querying file (WeaponExcelConfigData, for example), optional field names, and one or two pattern arguments
```
WeaponExcelConfigData.nameTextMapHash ?x ?y
---- file name ------|-- field name -|-- two pattern arguments 
```
A pattern argument can be a variable (starts with a `?`), or a constant. When it's a variable, it means the variable will match anything.  
The above example will output all items in WeaponExcelConfigData with `?x` constrained to the item and `?y` constrained to the nameTextMapHash.

Another example takes only one pattern argument
```
WeaponExcelConfigData ?x
```
where `?x` will be constrained to any item in the file

#### Constants
arguments can be constants
```
WeaponExcelConfigData.weaponType ?x "WEAPON_SWORD_ONE_HAND" 
```
Will output all weapons that `weaponType` equals to `WEAPON_SWORD_ONE_HAND`  
Note the strings are quoted.

arguments can also be numbers
```
WeaponExcelConfigData.id ?x 11101
```
### Text Map Query
Text Maps consist of only number keys and string values.
You can query as:
```
CHS ?key ?value
```
Text Maps are large in entries count, so it's better not to use two variables at the same time for performance.
### Functional Query
some functions are built-in, for example, the `split_by a b c d` ensures that `a || b || c == d` (`||` means string concatenation)
```
split_by ?x ?y "c" "abc"
```
produces:
```
[
  {
    "?x": "",
    "?y": "ab"
  },
  {
    "?x": "a",
    "?y": "b"
  },
  {
    "?x": "ab",
    "?y": ""
  }
]
```
It's convenient to remove some common prefix/postfix in some field, for example:
```
WeaponExcelConfigData.nameTextMapHash ?x ?y && CHS ?y "祭礼剑" && WeaponExcelConfigData.icon ?x ?icon && split_by "" "UI_EquipIcon_" ?iconname ?icon
```
This extracts the icon name removing the common prefix `UI_EquipIcon_`
### Compound Query
`&&` and `||` are used to form compound queries, as is already shown in previous examples

## Pitfalls
The query complexity will grow in exponential with respect to variable count in the worst case.  
It's better to not use too much variables

### Query Order Matters
Orders will affect performance
```
// this is fast
CHS ?y ?chs
    && split_by ?prefix "岩" ?postfix ?chs
    && WeaponExcelConfigData.nameTextMapHash ?x ?y
    && WeaponExcelConfigData.icon ?x ?icon
    && split_by "" "UI_EquipIcon_" ?iconname ?icon

// this is slow
WeaponExcelConfigData.nameTextMapHash ?x ?y
    && CHS ?y ?chs
    && split_by ?prefix "岩" ?postfix ?chs
    && WeaponExcelConfigData.icon ?x ?icon
    && split_by "" "UI_EquipIcon_" ?iconname ?icon
```

## Examples
Extract a Weapon that CHS name is "黑剑"
```
WeaponExcelConfigData.nameTextMapHash ?weapon ?hash && CHS ?hash "黑剑"
```


Get all weapons where there is "Sword" or "sword" in the EN name
```
EN ?y ?en
    && (split_by ?prefix "Sword" ?postfix ?en || split_by ?prefix "sword" ?postfix ?en)
    && WeaponExcelConfigData.nameTextMapHash ?x ?y
```


Get all characters whose quality is purple and weapon type is sword
```
AvatarExcelConfigData.qualityType ?x "QUALITY_PURPLE"
    && AvatarExcelConfigData.weaponType ?x "WEAPON_SWORD_ONE_HAND"
```


Get all girl characters
```
AvatarExcelConfigData.bodyType ?x "BODY_GIRL"
    || AvatarExcelConfigData.body ?x "BODY_LADY"
    || AvatarExcelConfigData.body ?x "BODY_LOLI"
```