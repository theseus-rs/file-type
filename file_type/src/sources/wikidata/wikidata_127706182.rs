use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127706182: FileFormat = FileFormat {
    id: 127_706_182,
    source_type: SourceType::Wikidata,
    name: "Less file format",
    extensions: &["less"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
