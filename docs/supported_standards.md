# Condensed List

This crate will try to apply all the following standards to the given street address(es). The user can choose to ignore any of these standards.

* 212 - All uppercase letters
* 213 - Use official abbreviations for sceondary address designators
* 213.1 - All common designators use official abbreviations (see `pub28_c2.csv`)
* 213.2 - Space between pound (#) sign and secondary number
* 222 - Remove punctuation
* 223 - Replace N/S/E/W with NORTH/SOUTH/EAST/WEST in city names
* 232 - Remove hyphens from street and city names
* 233.1 - Use official one- or two-letter abbreviation for the 8 directionals
* 233.23 - If two directionals appear side by side in a street, spell them out fully (eg SOUTHWEST or NORTH EAST)
* 233.3 - If a directional appears between the street name and suffix, spell it out (eg BAY WEST DR, NORTH AVE)
* 234.1 - Use official abbreviations for suffixes
* 234.2 - If two suffixes appear side by side, abbreviate the last one but spell out the first one
* 241 - Abbreviate rural route addresses to RR ## BOX ##
* 242 - Remove leading zero from RR number
* 244 - Change RFD (rural-free delivery) and RD (rural delivery) designations to RR
* 245 - Remove all other address designations form a rural route address (other than RR ## BOX ##)
* 251 - Abbreviate highway contract route addresses to HC ## BOX ##
* 252 - Remove leading zero from HC number
* 253 - Abbreviate STAR ROUTE as HC
* 254 - Remove all other address designations from a highway contract route address (other than HC ## BOX ##)
* 281 - Abbreviate PO BOX addresses to PO BOX ###
* 282 - If a PO BOX number starts with a hyphen, replace it with a zero
* 283 - Replace CALLER, FIRM CALLER, BIN, LOCKBOX, or DRAWER with PO BOX
* 284 - If a pound (#) sign appears after a street suffix followed by a number, replace the pound sign with UNIT

# Detailed List

### 211 Standardized Delivery Address Line and Last Line

The Delivery Address Line and the Last Line of addresses output to the mailpiece should be complete, standardized, and validated with the ZIP+4 file and City State file, respectively.

The Postal Service defines a _complete address_ as one that has all the address elements necessary to allow an exact match with the current Postal Service ZIP+4 and City State files to obtain the finest level of ZIP+4 and delivery point codes for the delivery address. A complete address may be required on mail at some automation rates. See the DMM for more detailed information.

A _standardized address_ is one that includes all required address elements and that uses the Postal Service standard abbreviations (as shown in this publication or in the current Postal Service ZIP+4 file).

### 212 Format

Uppercase letters are preferred on all lines of the address block.

### 213 Secondary Address Unit Designators

Secondary address unit designators, such as _APARTMENT_ or _SUITE,_ are required to be printed on the mailpiece for address locations containing secondary unit designators. The preferred location is at the end of the Delivery Address Line. The pound sign (#) should not be used as a secondary unit designator if the correct designation, such as _APT_ or _STE,_ is known or is shown in the ZIP+4 file.

#### 213.1 Common Designators

*See pub28_c2.csv*
#### 213.2 Pound Sign (#)

If the pound sign (#) is used, there must be a space between the pound sign and the secondary number.

### 222 Punctuation

With the exception of the hyphen in the ZIP+4 Code, punctuation may be omitted in the delivery address block.

### 223 Spelling of City Names

Spell city names in their entirety. When abbreviations must be used due to labeling constraints, use only the approved 13–character abbreviations provided in the City State file.

*Only replace standalone W N S E characters in the City element with WEST NORTH SOUTH EAST, respectively*

### 231 Components

The Delivery Address Line, as matched against the ZIP+4 file, must be broken down into its individual components on the mailpiece with one space between address elements.

These components are the primary address number, predirectional, street name, suffix, postdirectional, secondary address identifier, and secondary address.

The Postal Service uses the parsing logic below to enter address information into the files. When parsing the Delivery Address Line into the individual components, start from the right–most element of the address and work toward the left. Place each element in the appropriate field until all address components are isolated. This process facilitates matching files with AIS products and produces the correct format for output to a mailpiece.

![Delivery address may contain proper abbreviations.](https://pe.usps.com/text/pub28/images/28c2_012_1.jpg "Delivery address may contain proper abbreviations.")

### 232 Street Name

Information found in the street name field of the ZIP+4 file is used as the street name. The ZIP+4 file indicates the preferred primary street name to ensure that the correctly designated primary street record is matched during the address standardization processes.

Punctuation is normally limited to periods, slashes, and hyphens:

- Periods: 39.2 RD
- Slashes (fractional addresses): 33 1/2 AVE
- Hyphens (hyphenated addresses): 289-01 MID-ISLAND PLZ

![Use hyphens in ranges. Do not use hyphens in street names.](https://pe.usps.com/text/pub28/images/28c2_013_1.jpg "Use hyphens in ranges. Do not use hyphens in street names.")

Note: Hyphens in the address range are significant and are not removed. Hyphens in the street or city name, however, normally are not significant and may be replaced with a space.

### 233 Directionals

This is a term the Postal Service uses to refer to the part of the address that gives directional information for delivery (i.e., N, S, E, W, NE, NW, SE, SW).

#### 233.1 Abbreviations

Abbreviate directionals (if they are one of the eight standard directionals listed in AIS files) to the appropriate one– or two–character abbreviation.

![Use directional abbreviations.](https://pe.usps.com/text/pub28/images/28c2_014_1.jpg "Use directional abbreviations.")

#### 233.2 Single Directionals

##### 233.21 Predirectional Field

When parsing the address from right to left, if a directional word is found as the first word in the street name and there is no other directional to the left of it, abbreviate it and locate it in the predirectional field of the ZIP+4 file for standardization purposes.

![Abbreviate directionals prior to street name.](https://pe.usps.com/text/pub28/images/28c2_014_2.jpg "Abbreviate directionals prior to street name.")

##### 233.22 Postdirectional Field

When parsing from right to left, if a directional word is located to the right of the street name and suffix, abbreviate it and locate it in the postdirectional field.

![Abbreviate directionals after the street name.](https://pe.usps.com/text/pub28/images/28c2_014_3.jpg "Abbreviate directionals after the street name.")

##### 233.23 Two Directionals

When two directional words appear consecutively as one or two words, before the street name or following the street name or suffix, then the two words become either the pre– or the postdirectionals. Exceptions are any combinations of _NORTH-SOUTH_ or _EAST–WEST_ as consecutive words. In these cases the second directional becomes part of the street name and is spelled out completely in the street name field.

![Spell out directionals if part of the delivery address name.](https://pe.usps.com/text/pub28/images/28c2_014_4.jpg "Spell out directionals if part of the delivery address name.")

The other exception is when the local address information unit has determined that one of the directional letters is used as an alphabet indicator and not as a directional.

![Do not combine directional letter with alphabet indicator.](https://pe.usps.com/text/pub28/images/28c2_014_5.jpg "Do not combine directional letter with alphabet indicator.")

Note: In this example, the two–word directional is the primary street name.

![Spell out the directional street name. Abbreviate directional after street name.](https://pe.usps.com/text/pub28/images/28c2_014_6.jpg "Spell out the directional street name. Abbreviate directional after street name.")

#### 233.3 Directional as Part of Street Name

When parsing from right to left, if the directional word appears between the street name and the suffix, then it appears as part of the street name spelled out in the ZIP+4 file and is spelled out on the mailpiece.

![Spell out the directional words between the street name and suffix.](https://pe.usps.com/text/pub28/images/28c2_014_7.jpg "Spell out the directional words between the street name and suffix.")

The exception is when the local AIS unit has determined that the letters (E, N, S, or W) are used as alphabet indicators and not as directionals.

![Do not spell out alphabet indicators as directional letters.](https://pe.usps.com/text/pub28/images/28c2_014_8.jpg "Do not spell out alphabet indicators as directional letters.")


### 234 Suffixes

#### 234.1 Abbreviations

The suffix of the address should conform to the standard suffix abbreviations listed in the ZIP+4 file (see Appendix [C](https://pe.usps.com/text/pub28/28apc_001.htm#ep538615)).

#### 234.2 Two Suffixes

If an address has two consecutive words that appear on the suffix table (Appendix [C](https://pe.usps.com/text/pub28/28apc_001.htm#ep538615)), abbreviate the second of the two words according to the suffix table and place it in the suffix field. The first of the two words is part of the street name. Spell it out on the mailpiece in its entirety after the street name.

![Abbreviate only the second of the two word suffixes.](https://pe.usps.com/text/pub28/images/28c2_015_1.jpg "Abbreviate only the second of the two word suffixes.")


## 24 Rural Route Addresses

### 241 Format

The rural route number on a mailpiece must be standardized as follows: RR ## BOX ## (in this example, “##” indicates the actual number or numbers).

![Sample rural route addresses.](https://pe.usps.com/text/pub28/images/28c2_021_1.jpg "Sample rural route addresses.")

Note: Do not use the words RURAL, NUMBER, NO., or the pound sign (#).

### 242 Leading Zero

A leading zero before the rural route number is not necessary.

![Do not insert zeros in front of rural route numbers.](https://pe.usps.com/text/pub28/images/28c2_022_1.jpg "Do not insert zeros in front of rural route numbers.")

### 244 Designations RFD and RD

Change the designations _RFD_ and _RD_ (as a meaning for rural or rural free delivery) to _RR_.

![Use RR in place of RFD or RD for rural or rural free delivery.](https://pe.usps.com/text/pub28/images/28c2_024_1.jpg "Use RR in place of RFD or RD for rural or rural free delivery.")

### 245 Additional Designations

There should be no additional designations, such as town or street names, on the Delivery Address Line of rural route addresses. Because street names used together with route and box numbers can create potential matching difficulty, mailers are encouraged to use only one style of addressing. If secondary name information is used, however, place it above the Delivery Address Line.

![Use only the RR/Box information on the delivery address line.](https://pe.usps.com/text/pub28/images/28c2_025_1.jpg "Use only the RR/Box information on the delivery address line.")

## 25 Highway Contract Route Addresses

### 251 Format

The highway contract route on a mailpiece must be standardized as follows: HC ## BOX ## (in this example, “##” indicates the actual number, numbers, or number/letter combinations).

![Use HC abbreviation for highway contract. Eliminate route/number.](https://pe.usps.com/text/pub28/images/28c2_027_1.jpg "Use HC abbreviation for highway contract. Eliminate route/number.")

Note: Do not use the words HIGHWAY CONTRACT, ROUTE, NUMBER, NO., STAR ROUTE, or the pound sign (#).


### 252 Leading Zero

A leading zero before the highway contract route number is not needed.

![Eliminate preceding zeros on highway contract numbers.](https://pe.usps.com/text/pub28/images/28c2_028_1.jpg "Eliminate preceding zeros on highway contract numbers.")


### 253 Star Route Designations

Change the designation _STAR ROUTE,_ which usually refers to highway contract route, to _HC._

![Use HC abbreviation for star route.](https://pe.usps.com/text/pub28/images/28c2_029_1.jpg "Use HC abbreviation for star route.")

### 254 Additional Designations

There should be no additional designations, such as town or street names, on the Delivery Address Line of highway contract route addresses. Street names used together with route and box numbers can create potential matching difficulty. Mailers are encouraged to use only one style of addressing. If secondary name information is used, however, place it above the Delivery Address Line.

![Do not use any additional designations when address is a star route.](https://pe.usps.com/text/pub28/images/28c2_030_1.jpg "Do not use any additional designations when address is a star route.")

## 28 Post Office Box Addresses

### 281 Format

The Post Office Box address on a mailpiece must be standardized as follows: PO BOX ## (in this example, “##” indicates the actual number, numbers, or letter).

![PO Box NN.](https://pe.usps.com/text/pub28/images/28c2_036_1.jpg "PO Box NN.")

### 282 Leading Zero

Post Office Box numbers that are preceded by significant leading zeroes are identified in the ZIP+4 file by a hyphen (–) preceding the box number. Convert the hyphen into a zero on the output mailpiece.

![Convert hyphen to preceding zeros when representing significant leading zeros in the PO box number.](https://pe.usps.com/text/pub28/images/28c2_037_1.jpg "Convert hyphen to preceding zeros when representing significant leading zeros in the PO box number.")

### 283 Designations

PO Box addresses often appear with the word _CALLER, FIRM CALLER, BIN, LOCKBOX,_ or _DRAWER._ Change these to _PO BOX_ as output on a mailpiece.

![Change Caller, Firm Caller, Bin, Lockbox, Drawer to standard PO box address.](https://pe.usps.com/text/pub28/images/28c2_038_1.jpg "Change Caller, Firm Caller, Bin, Lockbox, Drawer to standard PO box address.")

### 284 PO Box Street Addressing

PO Box services in some locations allow for an option to use the Post Office street address for the mailing address, along with the PO Box number preceded by a “#” sign or “UNIT” designation.

![Print mailstop code above recipient line.](https://pe.usps.com/text/pub28/images/28c2_039_1.jpg "Print mailstop code above recipient line.")

