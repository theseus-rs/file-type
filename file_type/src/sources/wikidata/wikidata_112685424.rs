use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112685424: FileFormat = FileFormat {
    id: 112_685_424,
    source_type: SourceType::Wikidata,
    name: "3D Studio (DOS) project-file format",
    extensions: &["prj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
