use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130245180: FileFormat = FileFormat {
    id: 130_245_180,
    source_type: SourceType::Wikidata,
    name: "LLVM assembly code file",
    extensions: &["ll"],
    media_types: &["text/x-llvm"],
    signatures: &[],
    related_formats: &[],
};
