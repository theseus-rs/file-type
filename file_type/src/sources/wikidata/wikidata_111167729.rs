use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111167729: FileFormat = FileFormat {
    id: 111_167_729,
    source_type: SourceType::Wikidata,
    name: "ACD/HNMR Calculated Spectrum file",
    extensions: &["hsp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
