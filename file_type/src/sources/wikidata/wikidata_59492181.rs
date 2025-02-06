use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59492181: FileFormat = FileFormat {
    id: 59_492_181,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Catalog (Windows)",
    extensions: &["sas7bcat", "sc7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
