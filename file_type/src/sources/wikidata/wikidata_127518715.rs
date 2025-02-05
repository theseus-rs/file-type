use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127518715: FileFormat = FileFormat {
    id: 127_518_715,
    source_type: SourceType::Wikidata,
    name: "Zephir source code file",
    extensions: &["zep"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
