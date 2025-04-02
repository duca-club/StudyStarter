# Study Starter
The tool for kick starting your studies at the beginning of a new trimester at Deakin.


State: <u>Pre-Alpha</u>
<br>

## What Is StudyStarter?
StudyStarter is a CLI tool that lets unit select the units they are doing for the trimester. It will create the file tree for said units; this includes individual unit folders and assignment folders. It will also generate student made README file for each unit (Assignment READMEs coming soon); These README files will include your class timetables, unit descriptions, assignment descriptions written by students that detail the "actual" task requirements and recommended actions (reduce the amount of resubmitting students have to do), and any other critical hard to find information.

## Why StudyStarter?
The Purpose of StudyStarter is to bring students up to pace with what they need to do in 5 minutes instead of 5 hours. The information provided by StudyStarter should be no more than a day out of date ever, so that students are never working on an inaccurate assumption.

## How to use StudyStarter
First download the latest version of StudyStarter from the release section on the Github repository (Add to path if you want it accessible everywhere).

Then run the executable with the units that you want (See unit_manifests for a list of available units) and the output directory:
```
[StudyStarter executable] -c SIT221 SIT202 SIT384 -o Bachelor_of_something
```

## Contributing
StudyStarter would not be possible without student contributions. We need students to update the manifest files with the latest assignment numbers, unit codes, and write accurate README files. If you would like to contribute, no programming knowledge is required. Simply submit your changes to the manifests and wait for it to be verified by other students. For instructions on how to write manifest.txt files, see the instructions for dirby: https://github.com/GhostUponAvon/dirby. If you would like to contribute to the code feel free to do so. If you would like to propose some ideas please join the DUCA Discord Server to communicate your ideas: https://discord.gg/7JPQk4xWCM.


## Credits And License
Based on dirby licensed under AGPL-3.0 by GhostUponAvon: https://github.com/GhostUponAvon/dirby