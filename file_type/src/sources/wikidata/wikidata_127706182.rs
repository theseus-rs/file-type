use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127706182: FileFormat = FileFormat {
    id: 127_706_182,
    source_type: SourceType::Wikidata,
    name: "Less file format",
    extensions: &["less"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
