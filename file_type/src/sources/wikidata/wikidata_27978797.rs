use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27978797: FileFormat = FileFormat {
    id: 27_978_797,
    source_type: SourceType::Wikidata,
    name: "Spectrum 512 Compressed",
    extensions: &["spc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
