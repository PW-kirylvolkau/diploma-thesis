import { Button, Card, CardActions, CardMedia, Typography } from "@mui/material";

export default function CourseElement({ courseName, courseImage }) {
    
    return (
      <Card sx={{ maxWidth: 450, minHeight: 250 }}>
        <CardMedia 
          component='img'
          alt={courseName}
          height="200px"
          image={courseImage}
        />
        <Typography gutterBottom variant="h5" component="div" sx={{ textAlign: "center", marginTop: 1 }}>
            {courseName}
        </Typography>
        <CardActions sx={{ float: "right", minWidth: "100%" }}>
            <Button size="medium" sx={{ minWidth: "50%", float: "left" }}>Learn More</Button>
            <Button size="medium" sx={{ minWidth: "50%" }}>Enroll</Button>
        </CardActions>
      </Card>
    );

}