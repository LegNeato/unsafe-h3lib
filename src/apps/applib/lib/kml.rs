use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn radsToDegs(radians: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellBoundary {
    pub numVerts: libc::c_int,
    pub verts: [LatLng; 10],
}
#[no_mangle]
pub unsafe extern "C" fn kmlPtsHeader(
    mut name: *const libc::c_char,
    mut desc: *const libc::c_char,
) {
    printf(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"<kml xmlns=\"http://www.opengis.net/kml/2.2\" xmlns:gx=\"http://www.google.com/kml/ext/2.2\" xmlns:kml=\"http://www.opengis.net/kml/2.2\" xmlns:atom=\"http://www.w3.org/2005/Atom\">\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"<Document>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"        <name>%s</name>\n\0" as *const u8 as *const libc::c_char,
        name,
    );
    printf(
        b"        <description>%s</description>\n\0" as *const u8 as *const libc::c_char,
        desc,
    );
    printf(b"        <Style id=\"s_circle_hl\">\n\0" as *const u8 as *const libc::c_char);
    printf(b"                <IconStyle>\n\0" as *const u8 as *const libc::c_char);
    printf(b"                        <scale>1.3</scale>\n\0" as *const u8 as *const libc::c_char);
    printf(b"                        <Icon>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                                <href>http://maps.google.com/mapfiles/kml/shapes/placemark_circle.png</href>\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"                        </Icon>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                        <hotSpot x=\"20\" y=\"2\" xunits=\"pixels\" yunits=\"pixels\"/>\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"                </IconStyle>\n\0" as *const u8 as *const libc::c_char);
    printf(b"                <LabelStyle>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                        <color>ff0000ff</color>\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b"                        <scale>2</scale>\n\0" as *const u8 as *const libc::c_char);
    printf(b"                </LabelStyle>\n\0" as *const u8 as *const libc::c_char);
    printf(b"        </Style>\n\0" as *const u8 as *const libc::c_char);
    printf(b"        <StyleMap id=\"m_ylw-pushpin\">\n\0" as *const u8 as *const libc::c_char);
    printf(b"                <Pair>\n\0" as *const u8 as *const libc::c_char);
    printf(b"                        <key>normal</key>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                        <styleUrl>#s_circle</styleUrl>\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"                </Pair>\n\0" as *const u8 as *const libc::c_char);
    printf(b"                <Pair>\n\0" as *const u8 as *const libc::c_char);
    printf(b"                        <key>highlight</key>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                        <styleUrl>#s_circle_hl</styleUrl>\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"                </Pair>\n\0" as *const u8 as *const libc::c_char);
    printf(b"        </StyleMap>\n\0" as *const u8 as *const libc::c_char);
    printf(b"        <Style id=\"s_circle\">\n\0" as *const u8 as *const libc::c_char);
    printf(b"                <IconStyle>\n\0" as *const u8 as *const libc::c_char);
    printf(b"                        <scale>1.1</scale>\n\0" as *const u8 as *const libc::c_char);
    printf(b"                        <Icon>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                                <href>http://maps.google.com/mapfiles/kml/shapes/placemark_circle.png</href>\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"                        </Icon>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                        <hotSpot x=\"20\" y=\"2\" xunits=\"pixels\" yunits=\"pixels\"/>\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"                </IconStyle>\n\0" as *const u8 as *const libc::c_char);
    printf(b"                <LabelStyle>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                        <color>ff000fff</color>\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b"                        <scale>2</scale>\n\0" as *const u8 as *const libc::c_char);
    printf(b"                </LabelStyle>\n\0" as *const u8 as *const libc::c_char);
    printf(b"        </Style>\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn kmlBoundaryHeader(
    mut name: *const libc::c_char,
    mut desc: *const libc::c_char,
) {
    printf(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"<kml xmlns=\"http://earth.google.com/kml/2.1\">\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b"<Folder>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"   <name>%s</name>\n\0" as *const u8 as *const libc::c_char,
        name,
    );
    printf(
        b"   <description>%s</description>\n\0" as *const u8 as *const libc::c_char,
        desc,
    );
    printf(b"   <Style id=\"lineStyle1\">\n\0" as *const u8 as *const libc::c_char);
    printf(b"      <LineStyle id=\"lineStyle2\">\n\0" as *const u8 as *const libc::c_char);
    printf(b"         <color>ff000fff</color>\n\0" as *const u8 as *const libc::c_char);
    printf(b"         <width>2</width>\n\0" as *const u8 as *const libc::c_char);
    printf(b"      </LineStyle>\n\0" as *const u8 as *const libc::c_char);
    printf(b"   </Style>\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn kmlPtsFooter() {
    printf(b"</Document>\n\0" as *const u8 as *const libc::c_char);
    printf(b"</kml>\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn kmlBoundaryFooter() {
    printf(b"</Folder>\n\0" as *const u8 as *const libc::c_char);
    printf(b"</kml>\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn outputLngLatKML(mut g: *const LatLng) {
    printf(
        b"            %8lf,%8lf,5.0\n\0" as *const u8 as *const libc::c_char,
        radsToDegs((*g).lng),
        radsToDegs((*g).lat),
    );
}
#[no_mangle]
pub unsafe extern "C" fn outputPointKML(mut g: *const LatLng, mut name: *const libc::c_char) {
    printf(b"<Placemark>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"   <name>%s</name>\n\0" as *const u8 as *const libc::c_char,
        name,
    );
    printf(b"   <styleUrl>#m_ylw-pushpin</styleUrl>\n\0" as *const u8 as *const libc::c_char);
    printf(b"   <Point>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"      <altitudeMode>relativeToGround</altitudeMode>\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"      <coordinates>\n\0" as *const u8 as *const libc::c_char);
    outputLngLatKML(g);
    printf(b"      </coordinates>\n\0" as *const u8 as *const libc::c_char);
    printf(b"   </Point>\n\0" as *const u8 as *const libc::c_char);
    printf(b"</Placemark>\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn outputTriKML(
    mut v1: *const LatLng,
    mut v2: *const LatLng,
    mut v3: *const LatLng,
    mut name: *const libc::c_char,
) {
    printf(b"<Placemark>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"<name>%s</name>\n\0" as *const u8 as *const libc::c_char,
        name,
    );
    printf(b"      <styleUrl>#lineStyle1</styleUrl>\n\0" as *const u8 as *const libc::c_char);
    printf(b"      <LineString>\n\0" as *const u8 as *const libc::c_char);
    printf(b"         <tessellate>1</tessellate>\n\0" as *const u8 as *const libc::c_char);
    printf(b"         <coordinates>\n\0" as *const u8 as *const libc::c_char);
    outputLngLatKML(v1);
    outputLngLatKML(v2);
    outputLngLatKML(v3);
    outputLngLatKML(v1);
    printf(b"         </coordinates>\n\0" as *const u8 as *const libc::c_char);
    printf(b"      </LineString>\n\0" as *const u8 as *const libc::c_char);
    printf(b"</Placemark>\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn outputBoundaryKML(
    mut b: *const CellBoundary,
    mut name: *const libc::c_char,
) {
    let mut v: *const LatLng = &(*b).verts as *const [LatLng; 10] as *const LatLng;
    outputPolyKML(v, (*b).numVerts, name);
}
#[no_mangle]
pub unsafe extern "C" fn outputPolyKML(
    mut geoVerts: *const LatLng,
    mut numVerts: libc::c_int,
    mut name: *const libc::c_char,
) {
    printf(b"<Placemark>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"<name>%s</name>\n\0" as *const u8 as *const libc::c_char,
        name,
    );
    printf(b"      <styleUrl>#lineStyle1</styleUrl>\n\0" as *const u8 as *const libc::c_char);
    printf(b"      <LineString>\n\0" as *const u8 as *const libc::c_char);
    printf(b"         <tessellate>1</tessellate>\n\0" as *const u8 as *const libc::c_char);
    printf(b"         <coordinates>\n\0" as *const u8 as *const libc::c_char);
    let mut v: libc::c_int = 0 as libc::c_int;
    while v < numVerts {
        outputLngLatKML(&*geoVerts.offset(v as isize));
        v += 1;
    }
    outputLngLatKML(&*geoVerts.offset(0 as libc::c_int as isize));
    printf(b"         </coordinates>\n\0" as *const u8 as *const libc::c_char);
    printf(b"      </LineString>\n\0" as *const u8 as *const libc::c_char);
    printf(b"</Placemark>\n\0" as *const u8 as *const libc::c_char);
}
