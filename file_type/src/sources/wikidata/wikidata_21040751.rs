use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_21040751: FileFormat = FileFormat {
    id: 21_040_751,
    source_type: SourceType::Wikidata,
    name: "Farandole Composer format",
    extensions: &["far"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
