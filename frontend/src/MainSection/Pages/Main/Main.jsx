import { Grid } from "@mui/material";
import CourseElement from "./CourseElement/CourseElement";

export default function Main() {
   
    const coursesList = data.map(d => ( 
          <Grid 
            item 
            xs={4} 
            key={d.id}
            sx={{ marginBottom: 2, minWidth: "300px"}}
          >
            <CourseElement 
              courseImage={d.courseImage} 
              courseName={d.courseName}
            />
          </Grid>
        ));

    return (
      <>
        <Grid container spacing={2} sx={{ minWidth: "100vw", padding: "0 5vw"}}>
          {coursesList}
        </Grid>
      </>
    );

}

const data = [ 
        {
            id: 1,
            courseName: "Maths", 
            courseImage: "https://www.fife.ac.uk/media/3828/mathematics-png.jpg?anchor=center&mode=crop&width=1200&height=800&rnd=132155248940000000"
        },
        {
            id: 2,
            courseName: "Biology",
            courseImage: "https://img.freepik.com/free-vector/science-biology-scribbles_23-2147501586.jpg?w=740&t=st=1673179113~exp=1673179713~hmac=c9ee7bc0b769126e229a65a3490b7b79772888f43ba20e788c46700a483e0f0a"
        },
         {
            id: 3,
            courseName: "Maths", 
            courseImage: "https://www.fife.ac.uk/media/3828/mathematics-png.jpg?anchor=center&mode=crop&width=1200&height=800&rnd=132155248940000000"
        },
        {
            id: 4,
            courseName: "Biology",
            courseImage: "https://img.freepik.com/free-vector/science-biology-scribbles_23-2147501586.jpg?w=740&t=st=1673179113~exp=1673179713~hmac=c9ee7bc0b769126e229a65a3490b7b79772888f43ba20e788c46700a483e0f0a"
        }
    ];