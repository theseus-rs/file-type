use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117224387: FileFormat = FileFormat {
    id: 117_224_387,
    source_type: SourceType::Wikidata,
    name: "TurboCAD for DOS 3.0 file",
    extensions: &["tcd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
