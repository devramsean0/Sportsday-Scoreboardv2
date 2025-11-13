# Configuring the Scoreboard
This is a rough overview of the configuration language/features. It was designed to be powerful yet easy to write.

## Keys
### version (required)
This key is the versioning for the config file. The application will only do a db reset + rebuild when this value changes.

### genders (required)
This key represents the valid genders for the system. This is where you define the allowed keys for events to restrict based on. This is also how the system calculates how many of each gendered event it needs.

### scores (required)
This key represents the valid score values for the system. It is an array and has a few values per score
#### name (required)
The human friendly name that shows up in the set score interface. A string
#### value (required)
The actual value of the score increase. A number

### years (required)
This key represents the valid years for the system. It helps it work out how many years of an event it needs to create.
#### id (required)
This is a internal name used to reference the year in individual events/through the API. A string
#### name (required)
This is the public showing name of the year. This reflects on the scoreboard & set score pages

### forms (required)
This key represents the valid forms for the system. It helps it work out how many forms for each event it needs to create.
#### id (required)
This is a internal name used to reference the form in individual events/through the API. A string
#### name (required)
This is the public showing name of the form. This reflects on the scoreboard & set score pages



