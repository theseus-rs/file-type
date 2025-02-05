use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130265836: FileFormat = FileFormat {
    id: 130_265_836,
    source_type: SourceType::Wikidata,
    name: "Linden Scripting Language source code file",
    extensions: &["lsl"],
    media_types: &["text/x-lsl"],
    signatures: &[],
    related_formats: &[],
};
