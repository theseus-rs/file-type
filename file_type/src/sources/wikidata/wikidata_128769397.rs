use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128769397: FileFormat = FileFormat {
    id: 128_769_397,
    source_type: SourceType::Wikidata,
    name: "Concise Data Definition Language file",
    extensions: &["cddl"],
    media_types: &["text/x-cddl"],
    internal_signatures: &[],
    related_formats: &[],
};
