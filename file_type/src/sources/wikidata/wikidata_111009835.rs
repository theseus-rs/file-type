use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009835: FileFormat = FileFormat {
    id: 111_009_835,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Post Card File format",
    extensions: &["pcr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
