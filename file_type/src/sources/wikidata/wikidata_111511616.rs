use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111511616: FileFormat = FileFormat {
    id: 111_511_616,
    source_type: SourceType::Wikidata,
    name: "SXG (ZX Spectrum) Graphic File",
    extensions: &["sxg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
