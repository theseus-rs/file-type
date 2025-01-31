use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127706182: FileFormat = FileFormat {
    id: 127_706_182,
    puid: "wikidata/127706182",
    name: "Less file format",
    extensions: &["less"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
