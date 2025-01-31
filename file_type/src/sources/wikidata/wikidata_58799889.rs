use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58799889: FileFormat = FileFormat {
    id: 58_799_889,
    puid: "wikidata/58799889",
    name: "PowerProject Teamplan",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
