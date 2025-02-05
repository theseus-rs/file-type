use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6026738: FileFormat = FileFormat {
    id: 6_026_738,
    source_type: SourceType::Wikidata,
    name: "PFB",
    extensions: &["pfb"],
    media_types: &["font/x-postscript-pfb"],
    signatures: &[],
    related_formats: &[],
};
