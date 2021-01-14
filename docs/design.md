# Quick Poll (2021-01-13)

 - [Development goals](#development-goals)
 - [Product goals](#product-goals)
 - [User Stories](#user-stories)
   - [Home page](#home-page)
   - [Configure categorical poll](#configure-categorical-poll)
   - [Configure numerical poll](#configure-numerical-poll)
   - [Create the poll](#create-poll)
   - [Categorical poll results](#categorical-poll-results)
   - [Numerical poll results](#numerical-poll-results)
   - [Respond to categorical poll](#respond-to-categorical-poll)
   - [Respond to numerical poll](#respond-to-numerical-poll)
   - [Submit poll response](#submit-poll-response)
 - [Technical Considerations](#technical-considerations)
   - [Generating QR codes](#generating-qr-codes)
   - [Cookies](#cookies)
   - [Categorical data format](#categorical-data-format)
   - [Numerical data format](#numerical-poll-results)
   - [Pages](#pages)
   - [Live data](#live-data)
   - [Presenting histograms](#presenting-histograms)
   - [Paths](#paths)
   - [Database](#database)
 - [Glossary](#glossary)

## Development Goals

 - Try a more traditional multi-page website as opposed to a single-page app
   powered by Vue. This may help see the pros and cons of using Vue (or any
   similar framework).
 - Configure Webpack manually and gain a better understanding of how it works.
 - Use SSE and see how they compare to WebSockets.
 - Use familiar technologies to do something a little different to the last
   project.
 - Create something that is just complex enough to be worth doing but not so
   complex that it takes months to complete.

## Product Goals

An easy to setup and easy to deploy poll. The poll is only one question for
simplicity. Responding to the poll is also very simple. Ideal usage is in a
classroom or theatre where participants can see a large screen showing a URL and
a QR code.

## User Stories

### Home page

The poll author navigates to the website. From this point they are given options
for things to create.
 - Create a categorical poll (choose between several options)
 - Create a numerical poll (choose a number in a range)

### Configure categorical poll

The poll author writes a question or a title for the poll. The author also adds
choices to the poll. There must be at least two choices. A choice is described
by a single piece of text. The length of the title and the length of the choices
may be limited. The number of choices may also be limited. The author must
decide whether participants will be allowed to choose multiple choices.

### Configure numerical poll

The author writes a question or a title for the poll. The author defines an
inclusive range. The range must be a valid range (the min cannot be greater than
the max). The range is optional. The min or max (or both) may be infinity. The
allowed responses may either be integers or reals. There must be at least two
valid values that fit the range.

### Create poll

After the poll has been configured, it can be created. This will take the poll
author to the poll running page. This page must include some information.
 - The question/title
 - A URL for participants to use to access the poll
 - A QR code containing the URL
 - The number of responses that have been submitted
 - The number of responses currently being decided (people on the poll response
   page).
 - A button to close the poll

The number of responses submitted and being decided will need to be updated
live.

### Categorical poll results

A histogram of the responses. For each choice, show choice title, the number of
people that chose that choice, and the percentage of people that chose that
choice. The total number of people that responded should also be shown.

The histogram should be represented in which each choice being a row. This
avoids many columns from being squashed on the screen. Scrolling horizontally
is unpleasant without a trackpad.

The author should be given the option of saving the results as a CSV file for
import into Excel. There should be a column for each possible choice. The first
row in each column should be the choice title. Each following row represents a
single response. A 1 in a cell means that the choice was present. A blank cell
means that the choice was not present. Here's an example.

https://libguides.library.kent.edu/SPSS/Multiple-Response-Sets

**Which of these to you own?**

| Laptop | Phone | Tablet |
|-------:|------:|-------:|
| 1      | 1     |        |
| 1      |       | 1      |
|        | 1     |        |

In mutually exclusive choice mode, the CSV will look a little different. Indexes
will be used. The responses will also be sorted. Here's another example.

**Which of these do you use most often?**

| Responses | Key    |
|----------:|--------|
| 0         | Laptop |
| 0         | Phone  |
| 1         | Tablet |
| 1         |        |
| 2         |        |

The CSV should contain the question so that all information is preserved. The
author has the option of returning to the home page to create another poll.

### Numerical poll results

The mean, minimum and maximum number chosen should be shown. In addition, the
median, interquartile range and standard deviation may also be useful. If there
are a small number of possible choices then a histogram may be appropriate. The
total number of people that responded should also be shown.

The author should be given the option of saving the results as a CSV file for
import into Excel. The numeric responses should in a single column and sorted.

The CSV should contain the question so that all information is preserved. The
author has the option of returning to the home page to create another poll.

### Respond to categorical poll

When the participant navigates to the URL, they are shown the title the list of
possible choices. The choices may either behave as radios or checkboxes. There
is a submit button to submit the response.

### Respond to numerical poll

When the participant navigates to the URL, they are shown the title and a number
box. If a min and max are defined then a range slider will also be shown. There
is a submit button to submit the response.

### Submit poll response

Once the response is submitted, a message indicating successful submission is
shown. No other actions are possible from this screen.

Participants may not complete the same poll multiple times. Attempting to do so
will show a message telling the user that this isn't allowed

## Technical Considerations

### Generating QR codes

 - Use a [service](http://goqr.me/api/)
 - Use a server-side [library](https://docs.rs/qrcode/0.12.0/qrcode/)

### Cookies

When the participant submits the poll response, a cookie is set so that they may
not complete the poll again.

The author will also need a session cookie. Only the author that created a poll
is allowed to run it or view its results. An author may create multiple polls if
they wish. However, polls are short-lived and will be deleted after some fixed
duration (perhaps 24 hours).

### Categorical data format

When multiple choices can be chosen, the response will be represented as a
bitset in a 32-bit integer. This would limit the total number of choices to 32
which is a reasonable limit

For simplicity, the same data format could be used in mutually exclusive choice
mode but this could make some operations less efficient than they might be if
they indexes were used. The CSV representation must also be considered. Using
the same CSV representation for both may be inconvenient.

The client-side JavaScript must be responsible for converting the form data to
this compact representation and submitting it to the server.

### Numerical data format

A response to a numerical poll should be a double-precision floating point
number.

### Pages

For something as simple as this, it probably doesn't matter whether I use Vue or
not. For this project. I won't use Vue. I'll still use Webpack though. I'll
configure it manually. I want to understand just how useful Vue was for the
previous project. Not using Vue means that it will probably be easier to create
separate pages for things. Obviously this is possible with vanilla JavaScript
but it can be a little tedious without using a framework that generates code.

### Live data

Since the statistics on the poll running page need to be updated live, SSE
should be used. Two-way communication is not required so WebSockets won't be
appropriate.

### Presenting histograms

Histograms could be rendered as DOM elements. A bar could be a div. The width
could be set as a percentage. This will probably require a lot of fiddling
around with CSS to position text in the right places but should be doable.

### Paths

 - `/` home page where polls can be created
 - `/configure/<type>` configure some type of poll
 - `/run/<unique_id>` run a poll
 - `/results/<unique_id>` results for a poll
 - `/respond/<unique_id>` respond to a poll

### Database

Since polls are meant to be short-lived and users don't need to create accounts,
a database might not be necessary. Keeping everything in-memory could be a bad
idea because if the server goes down then everything is lost.

## Glossary

 - **Categorical poll**: A type of poll that lets participants choose from a
   number of named choices. An example might be choosing between "beef", "pork"
   and "chicken".
 - **Numerical poll**: A type of poll that lets participants choose a number.
   This number may be restricted to a range. The number may be restricted to an
   integer. An example might choosing an integer between 1 and 5.
 - **Author**: A user that creates a poll.
 - **Participant**: A user that responds to a poll.
 - **Response**: A full response from a user. This may be a number from a
   numerical poll. This may also be one or more choices from a categorical poll.
 - **Choice**: One of the possible options that a participant is given when
   responding to a categorical poll.
