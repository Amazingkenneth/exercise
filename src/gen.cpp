#include "xpack/json.h"
#include <algorithm>
#include <codecvt>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <locale>
#include <filesystem>

const int Maxn = 10005;
int cnt, err, x, y, p;
char ch[Maxn];
std::vector<std::pair<int, int>> g[Maxn];
std::vector<std::string> idx;
std::string str, r[Maxn], e[Maxn];
struct Child {
  std::vector<int> att; // Attributes
  XPACK(O(att));
};
struct Node {
  std::string tp, zh, num;
  std::vector<Child> ch;
  XPACK(O(tp, zh, num, ch));
};
struct Index {
  std::vector<Node> idx;
  XPACK(O(idx));
} res;
int main() {
  using namespace std;
  //using namespace ns;
  freopen("./../data/graph.txt", "r", stdin);
  for (cnt = 1; scanf(" %[^\n]", ch) != EOF; ++cnt) {
    if (ch[0] == '#')
      continue;
    int n = strlen(ch), id = n;
    for (int e = n - 1; e >= 0; --e)
      if (ch[e] == '#')
        id = e;
    ch[id] = '\0';
    int s = sscanf(ch, "%d,%d %d*", &x, &y, &p);
    if (s < 0)
      break;
    else if (s == 2)
      g[x].emplace_back(make_pair(y, -1));
    else if (s == 3)
      g[x].emplace_back(make_pair(y, p));
    else
      printf("Error occurs on line %d , s = %d, (%d, %d, %d).\n", cnt, s, x, y,
             p),
          ++err;
  }
  cerr << "Successfully read " << cnt << " lines of the Graph, with " << err
       << " error" << (err > 1 ? "s." : ".") << endl;
  fclose(stdin);
  ifstream reader("./../data/string.txt");
  if (!reader.is_open()) {
    cerr << "Cannot open /data/string.txt" << endl;
    return -1;
  }
  for (cnt = 1; getline(reader, str); ++cnt) {
    if (str[0] == '#')
      continue;
    bool eng = false;
    printf("Line %d's length %d\n", cnt, str.size());
    for (auto c : str) {
      if (c == '|') {
        eng = true;
        continue;
      } else if (c == '#') {
        break;
      }
      if (eng)
        e[cnt].push_back(c);
      else
        r[cnt].push_back(c);
    }
  }
  reader.close();
  for (int i = 1; i <= cnt; ++i) {
    if (r[i].empty()) {
      cerr << "Node " << i << " has no content!" << endl;
      continue;
    }
    string name = "n" + to_string(i), tp;
    vector<Child> t;
    if (!g[i].empty()) {
      sort(g[i].begin(), g[i].end());
      if (g[i][0].second == -1) {
        tp = "Question";
        for (auto it : g[i]) {
          vector<int> in{it.first};
          t.emplace_back(Child{in});
        }
      } else {
        tp = "Random";
        for (auto it : g[i]) {
          vector<int> in{it.first, it.second};
          t.emplace_back(Child{in});
        }
      }
    } else {
      tp = "Ending";
    }
    res.idx.emplace_back(Node { tp, r[i], name, t });
  }
  std::ofstream o("map.json");
  string json = xpack::json::encode(res);
  o << json << std::endl;
  o.close();
  return 0;
}
