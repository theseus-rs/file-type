use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130265836: FileFormat = FileFormat {
    id: 130_265_836,
    puid: "wikidata/130265836",
    name: "Linden Scripting Language source code file",
    extensions: &["lsl"],
    media_types: &["text/x-lsl"],
    internal_signatures: &[],
    related_formats: &[],
};
