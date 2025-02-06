use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51331855: FileFormat = FileFormat {
    id: 51_331_855,
    source_type: SourceType::Wikidata,
    name: "Microsoft Powerpoint Design Template",
    extensions: &["pot"],
    media_types: &["application/mspowerpoint"],
    signatures: &[],
    related_formats: &[],
};
