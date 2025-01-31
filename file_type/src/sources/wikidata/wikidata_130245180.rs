use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130245180: FileFormat = FileFormat {
    id: 130_245_180,
    puid: "wikidata/130245180",
    name: "LLVM assembly code file",
    extensions: &["ll"],
    media_types: &["text/x-llvm"],
    internal_signatures: &[],
    related_formats: &[],
};
