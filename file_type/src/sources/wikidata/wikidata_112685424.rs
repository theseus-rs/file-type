use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112685424: FileFormat = FileFormat {
    id: 112_685_424,
    puid: "wikidata/112685424",
    name: "3D Studio (DOS) project-file format",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
