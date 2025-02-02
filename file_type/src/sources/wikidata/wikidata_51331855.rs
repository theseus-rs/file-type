use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51331855: FileFormat = FileFormat {
    id: 51_331_855,
    source_type: SourceType::Wikidata,
    name: "Microsoft Powerpoint Design Template",
    extensions: &["pot"],
    media_types: &["application/mspowerpoint"],
    internal_signatures: &[],
    related_formats: &[],
};
