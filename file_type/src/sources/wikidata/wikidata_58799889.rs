use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58799889: FileFormat = FileFormat {
    id: 58_799_889,
    source_type: SourceType::Wikidata,
    name: "PowerProject Teamplan",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
