use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_99850841: FileFormat = FileFormat {
    id: 99_850_841,
    source_type: SourceType::Wikidata,
    name: "Picture Publisher Bitmap 6-10",
    extensions: &["ppf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
