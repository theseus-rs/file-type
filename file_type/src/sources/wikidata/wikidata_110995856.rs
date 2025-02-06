use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110995856: FileFormat = FileFormat {
    id: 110_995_856,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Card File format",
    extensions: &["car"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
