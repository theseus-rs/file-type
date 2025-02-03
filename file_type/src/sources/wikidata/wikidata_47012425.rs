use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47012425: FileFormat = FileFormat {
    id: 47_012_425,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visual FoxPro Table file format",
    extensions: &["dbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
