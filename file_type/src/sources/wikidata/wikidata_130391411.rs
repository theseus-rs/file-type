use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130391411: FileFormat = FileFormat {
    id: 130_391_411,
    source_type: SourceType::Wikidata,
    name: "Objective-J source code file",
    extensions: &["j"],
    media_types: &["text/x-objective-j"],
    signatures: &[],
    related_formats: &[],
};
