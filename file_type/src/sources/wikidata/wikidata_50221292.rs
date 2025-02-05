use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50221292: FileFormat = FileFormat {
    id: 50_221_292,
    source_type: SourceType::Wikidata,
    name: "JEOL NMR Spectroscopy",
    extensions: &["jdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
