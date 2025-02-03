use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130478690: FileFormat = FileFormat {
    id: 130_478_690,
    source_type: SourceType::Wikidata,
    name: "Pike source code file",
    extensions: &["pike"],
    media_types: &["text/x-pike"],
    internal_signatures: &[],
    related_formats: &[],
};
