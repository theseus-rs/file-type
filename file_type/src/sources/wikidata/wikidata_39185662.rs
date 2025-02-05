use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_39185662: FileFormat = FileFormat {
    id: 39_185_662,
    source_type: SourceType::Wikidata,
    name: "AHK script",
    extensions: &["ahk"],
    media_types: &["text/x-autohotkey"],
    signatures: &[],
    related_formats: &[],
};
