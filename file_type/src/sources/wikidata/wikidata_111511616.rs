use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111511616: FileFormat = FileFormat {
    id: 111_511_616,
    source_type: SourceType::Wikidata,
    name: "SXG (ZX Spectrum) Graphic File",
    extensions: &["sxg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
