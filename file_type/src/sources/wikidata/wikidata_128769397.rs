use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128769397: FileFormat = FileFormat {
    id: 128_769_397,
    source_type: SourceType::Wikidata,
    name: "Concise Data Definition Language file",
    extensions: &["cddl"],
    media_types: &["text/x-cddl"],
    signatures: &[],
    related_formats: &[],
};
