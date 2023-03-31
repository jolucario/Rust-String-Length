///This code measure the length of the string. Which are EVERYTHING inside "" (except the "" themselves)
///Yeah that also includes the ``` since this is from a Discord message which are based on track listing 
///of the album The Stanley Parable: Ultra Deluxe – The Complete Soundtrack
///https://crowscrowscrows.bandcamp.com/album/the-stanley-parable-ultra-deluxe-the-complete-soundtrack
fn main() {
    let track_listing = "```markdown
1. Kevan Brighting - Intro 00:05
2. The Blake Robinson Synthetic Orchestra - Introducing Stanley 01:28
3. The Blake Robinson Synthetic Orchestra - Exploring Stanley 03:49
4. Yiannis Ioannides - Truth & Lies 00:46
5. Yiannis Ioannides - Control 04:29
6. Yiannis Ioannides - And Stanley Was Happy 01:51
7. The Blake Robinson Synthetic Orchestra - Following Stanley 01:31
8. Davey Wreden - 8 00:08
9. Christiaan Bakker - Showing Him The Door 04:06
10. Christiaan Bakker - Showing Him The Door (Panic) 00:22
11. The Blake Robinson Synthetic Orchestra - Contemplating Stanley 01:21
12. The Blake Robinson Synthetic Orchestra - Informing Stanley 01:38
13. Tom Schley - Creep Tower 01:37
14. Tom Schley - Thinking Theme 01:02
15. Dominik Johann - Good Job. You’ve Made It To The Bottom of The Mind Control Facility. Well Done. 01:11
16. The Blake Robinson Synthetic Orchestra - Anticipating Stanley 01:11
17. Tom Schley - The Memory Zone 03:36
18. Alina Johann, Titouan Millet, William Pugh - C’est la Chanson sur La Mémory Zone (en Français) 02:36
19. Tom Schley - New New Content 01:24
20. Tom Schley - The Stanley Parable 2 03:50
21. Tom Schley - The End Is Never The End Again 04:24
22. Tom Schley - Is This A Bucket? 03:47
23. Tom Schley - Stanley’s New Apartment 03:42
24. Chrstiaan Bakker, Tom Schley - Showing Him The Bucket 02:00
25. Tom Schley - The Bucket Destroyer Theme 01:16
26. Tom Schley - Gambhorra’ta, Treasurer of The Profaned Vault 02:32
27. Dominik Johann, Tom Schley - Home Sweet Home 02:12
28. Tom Schley - The Silly Birds Observation Facility 01:40
29. Tom Schley - But Nobody Came (not the one from undertale) 00:54
30. Tom Schley - Stanley's Sacrifice 02:32
31. Tom Schley - The Infinite Hole 03:20
32. William Pugh, Tom Schley - Falling Funk 02:13
33. Tom Schley - Figurine Finders Comittee 01:52
34. The Blake Robinson Synthetic Orchestra, Tom Schley - The End Was Never The End 01:37
35. Tom Schley - Epilogue 03:07
36. The Blake Robinson Synthetic Orchestra - Leaving Stanley 01:33
37. Tom Schley, William Pugh - Marketing Stanley 01:00
38. The Blake Robinson Synthetic Orchestra - Pondering Stanley 01:18
39. Christiaan Bakker - The Stanley Monologue (Teaser Music) 01:44
40. Christiaan Bakker - Not Enough Mana... I Mean Emotion! -_- (Teaser Music) 00:40
41. The Blake Robinson Synthetic Orchestra - Broadcasting Stanley 01:14
42. The Blake Robinson Synthetic Orchestra - Celebrating Stanley 00:20
43. The Blake Robinson Synthetic Orchestra - Educating Stanley 01:12
44. Christiaan Bakker - Narrated Wanderer (Unused) 02:16
45. Christiaan Bakker - IM CONFUS (Unused) 02:08
46. Tom Schley, Filomena Franke, Dominik Johann, William Pugh, Joe Finegold - Ambiences 22:22
```";

    let ptr = track_listing.as_ptr();
    let len = track_listing.len();
    
    print!("length: {}", len);
}