use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112685424: FileFormat = FileFormat {
    id: 112_685_424,
    source_type: SourceType::Wikidata,
    name: "3D Studio (DOS) project-file format",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
