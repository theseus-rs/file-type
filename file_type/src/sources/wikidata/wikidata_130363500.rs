use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130363500: FileFormat = FileFormat {
    id: 130_363_500,
    source_type: SourceType::Wikidata,
    name: "NCL file",
    extensions: &["ncl"],
    media_types: &["text/ncl"],
    signatures: &[],
    related_formats: &[],
};
