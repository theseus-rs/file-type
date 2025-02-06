use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117224387: FileFormat = FileFormat {
    id: 117_224_387,
    source_type: SourceType::Wikidata,
    name: "TurboCAD for DOS 3.0 file",
    extensions: &["tcd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
